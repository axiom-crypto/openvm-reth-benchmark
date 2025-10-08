import os
import subprocess
from pathlib import Path
from typing import Dict

from fastapi import FastAPI, HTTPException, Depends, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel

# import secrets
# from fastapi.security import HTTPBasic, HTTPBasicCredentials


# security = HTTPBasic()
# API_USERNAME = os.environ.get("API_USERNAME") or os.environ.get("BASIC_AUTH_USERNAME")
# API_PASSWORD = os.environ.get("API_PASSWORD") or os.environ.get("BASIC_AUTH_PASSWORD")


# def authenticate(credentials: HTTPBasicCredentials = Depends(security)):
#     if not API_USERNAME or not API_PASSWORD:
#         # Fail closed if server auth is not configured
#         raise HTTPException(status_code=500, detail="Server auth not configured")

#     is_user_ok = secrets.compare_digest(credentials.username, API_USERNAME)
#     is_pass_ok = secrets.compare_digest(credentials.password, API_PASSWORD)
#     if not (is_user_ok and is_pass_ok):
#         raise HTTPException(
#             status_code=status.HTTP_401_UNAUTHORIZED,
#             detail="Unauthorized",
#             headers={"WWW-Authenticate": "Basic"},
#         )

#     return credentials.username


# app = FastAPI(dependencies=[Depends(authenticate)])
app = FastAPI()


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
        status = f"{{Failed: {exit_code}}}"
    e2e_latency_ms = None
    latency_ms_path = j.job_dir / "latency_ms.txt"
    if os.path.exists(latency_ms_path):
        with open(latency_ms_path, "r") as f:
            e2e_latency_ms = int(f.read())

    return JSONResponse(
        status_code=200,
        content={
            "status": status,
            "num_instructions": 0,  # TODO: fix this
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
