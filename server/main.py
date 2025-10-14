import os
import subprocess
from pathlib import Path
from typing import Dict

from fastapi import FastAPI, HTTPException, Depends, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel


app = FastAPI()


def _run_s5cmd_copy(source_uri: str, destination_path: Path) -> None:
    destination_path.parent.mkdir(parents=True, exist_ok=True)
    args = ["s5cmd", "cp", source_uri, str(destination_path)]
    # Use a simple run that surfaces non-zero exit codes
    subprocess.run(args, check=True, text=True)


@app.on_event("startup")
def download_proving_keys_on_startup() -> None:
    app_pk_uri = os.environ.get("APP_PK_URI")
    agg_pk_uri = os.environ.get("AGG_PK_URI")
    if not app_pk_uri or not agg_pk_uri:
        raise ValueError("APP_PK_URI and AGG_PK_URI must be set")

    app_pk_path = Path(os.environ.get("APP_PK_PATH", "/app/app_pk"))
    agg_pk_path = Path(os.environ.get("AGG_PK_PATH", "/app/agg_pk"))

    # Download only if missing to keep startup idempotent
    try:
        if not app_pk_path.exists():
            _run_s5cmd_copy(app_pk_uri, app_pk_path)
        if not agg_pk_path.exists():
            _run_s5cmd_copy(agg_pk_uri, agg_pk_path)
    except Exception as e:  # Keep server up but surface the error in logs
        # Printing rather than logging to avoid adding a logger dependency
        print(f"[startup] failed to download proving keys: {e}", flush=True)


class Job:
    def __init__(
        self,
        pid: int,
        popen: subprocess.Popen,
        job_dir: Path,
        mode: str,
    ):
        self.pid = pid
        self.popen = popen
        self.job_dir = job_dir
        self.mode = mode


JOBS: Dict[str, Job] = {}


def run_proof(
    proof_uuid: str,
    stdout_path: Path,
    stderr_path: Path,
) -> subprocess.Popen:
    script_path = Path(__file__).parent / "prove_block.sh"
    if not script_path.exists():
        raise FileNotFoundError(f"Wrapper script not found at {script_path}")

    args = [str(script_path), proof_uuid]

    stdout_f = stdout_path.open("w")
    stderr_f = stderr_path.open("w")
    return subprocess.Popen(args, stdout=stdout_f, stderr=stderr_f, text=True)


class StartProofRequest(BaseModel):
    proof_uuid: str

@app.get("/healthz")
async def health():
    return JSONResponse(status_code=200, content={"status": "healthy"})

@app.post("/start_proof")
async def start_proof(req: StartProofRequest):
    proof_uuid = req.proof_uuid
    mode = "prove-stark"
    # If a job for this proof is already running, return its info
    j = JOBS.get(proof_uuid)
    if j and j.popen.poll() is None:
        return JSONResponse(
            status_code=200,
            content={
                "message": "job already running",
                "proof_uuid": proof_uuid,
                "pid": j.pid,
                "stdout_path": str(j.stdout_path),
                "stderr_path": str(j.stderr_path),
            },
        )

    jobs_root = Path(os.environ.get("JOBS_DIR", "/app/jobs"))
    jobs_root.mkdir(parents=True, exist_ok=True)
    job_dir = jobs_root / proof_uuid
    job_dir.mkdir(parents=True, exist_ok=True)

    stdout_path = job_dir / "stdout.log"
    stderr_path = job_dir / "stderr.log"

    try:
        proc = run_proof(proof_uuid, stdout_path, stderr_path)
    except FileNotFoundError as e:
        raise HTTPException(status_code=500, detail=str(e))

    JOBS[proof_uuid] = Job(proc.pid, proc, job_dir, mode)

    return JSONResponse(
        status_code=202,
        content={
            "message": "job started",
            "proof_uuid": proof_uuid,
            "pid": proc.pid,
            "job_dir": str(job_dir),
        },
    )


@app.get("/proof_state/{proof_uuid}")
async def status(proof_uuid: str):
    j = JOBS.get(proof_uuid)
    if not j:
        return JSONResponse(status_code=404, content={"error": "job not found"})
    exit_code = j.popen.poll()
    if exit_code is None:
        status = "InProgress"
    elif exit_code == 0:
        status = "Completed"
    else:
        status = "Failed"
    e2e_latency_ms = None
    latency_ms_path = j.job_dir / "latency_ms.txt"
    if os.path.exists(latency_ms_path):
        with open(latency_ms_path, "r") as f:
            e2e_latency_ms = int(f.read())

    state_instret_path = j.job_dir / "num_instret"
    if os.path.exists(state_instret_path):
        with open(state_instret_path, "r") as f:
            num_instret = int(f.read())
    else:
        num_instret = 0

    return JSONResponse(
        status_code=200,
        content={
            "status": status,
            "num_instructions": num_instret,
            "e2e_latency_ms": e2e_latency_ms,
        },
    )


@app.get("/logs")
async def logs(proof_uuid: str, n: int = 200):
    j = JOBS.get(proof_uuid)
    if not j:
        return JSONResponse(status_code=404, content={"error": "job not found"})

    def tail(path: Path, lines: int) -> list[str]:
        if not path.exists():
            return []
        with path.open("r") as f:
            data = f.readlines()
        return data[-lines:]

    return JSONResponse(
        status_code=200,
        content={
            "stdout": tail(j.job_dir / "stdout.log", n),
            "stderr": tail(j.job_dir / "stderr.log", n),
        },
    )
