import os
import shutil
import subprocess
from pathlib import Path
from typing import Dict

from fastapi import FastAPI, File, Form, HTTPException, UploadFile
from fastapi.responses import JSONResponse

app = FastAPI()


class Job:
    def __init__(
        self,
        pid: int,
        popen: subprocess.Popen,
        input_path: Path,
        stdout_path: Path,
        stderr_path: Path,
        mode: str,
    ):
        self.pid = pid
        self.popen = popen
        self.input_path = input_path
        self.stdout_path = stdout_path
        self.stderr_path = stderr_path
        self.mode = mode


JOBS: Dict[str, Job] = {}


def run_benchmark_binary(
    input_path: Path,
    block_number: str,
    mode: str,
    extra_args: list[str],
    stdout_path: Path,
    stderr_path: Path,
) -> subprocess.Popen:
    bin_path = os.environ.get("OVM_BIN", "/usr/local/bin/openvm-reth-benchmark-bin")
    if not Path(bin_path).exists():
        raise FileNotFoundError(f"Binary not found at {bin_path}")

    args = [
        bin_path,
        "--mode",
        mode,
        "--input-path",
        str(input_path),
        "--app-log-blowup",
        os.environ.get("APP_LOG_BLOWUP", "1"),
        "--leaf-log-blowup",
        os.environ.get("LEAF_LOG_BLOWUP", "1"),
        "--internal-log-blowup",
        os.environ.get("INTERNAL_LOG_BLOWUP", "2"),
        "--root-log-blowup",
        os.environ.get("ROOT_LOG_BLOWUP", "3"),
        "--block-number",
        block_number,
    ]
    args.extend(extra_args)

    stdout_f = stdout_path.open("w")
    stderr_f = stderr_path.open("w")
    return subprocess.Popen(args, stdout=stdout_f, stderr=stderr_f, text=True)


@app.post("/start_proof")
async def start_proof(
    block_number: str,
    file: UploadFile = File(
        ..., description="Input JSON produced by make_input or .bin"
    ),
    extra: str = Form("", description="Optional extra CLI args, space-separated"),
):
    mode = "prove-stark"
    # If a job for this block is already running, return its info
    j = JOBS.get(block_number)
    if j and j.popen.poll() is None:
        return JSONResponse(
            status_code=200,
            content={
                "message": "job already running",
                "block_number": block_number,
                "pid": j.pid,
                "stdout_path": str(j.stdout_path),
                "stderr_path": str(j.stderr_path),
            },
        )

    jobs_root = Path(os.environ.get("JOBS_DIR", "/app/jobs"))
    jobs_root.mkdir(parents=True, exist_ok=True)
    job_dir = jobs_root / block_number
    job_dir.mkdir(parents=True, exist_ok=True)

    # Save input to block-specific directory
    suffix = Path(file.filename or "input.json").suffix or ".json"
    input_path = job_dir / f"input{suffix}"
    with input_path.open("wb") as f:
        shutil.copyfileobj(file.file, f)

    stdout_path = job_dir / "stdout.log"
    stderr_path = job_dir / "stderr.log"

    try:
        extra_args = [arg for arg in extra.split() if arg]
        proc = run_benchmark_binary(
            input_path, block_number, mode, extra_args, stdout_path, stderr_path
        )
    except FileNotFoundError as e:
        raise HTTPException(status_code=500, detail=str(e))

    JOBS[block_number] = Job(proc.pid, proc, input_path, stdout_path, stderr_path, mode)

    return JSONResponse(
        status_code=202,
        content={
            "message": "job started",
            "block_number": block_number,
            "pid": proc.pid,
            "input_path": str(input_path),
            "stdout_path": str(stdout_path),
            "stderr_path": str(stderr_path),
        },
    )


@app.get("/status")
async def status(block_number: str):
    j = JOBS.get(block_number)
    if not j:
        return JSONResponse(status_code=404, content={"error": "job not found"})
    exit_code = j.popen.poll()
    return JSONResponse(
        status_code=200,
        content={
            "block_number": block_number,
            "pid": j.pid,
            "running": exit_code is None,
            "exit_code": exit_code,
            "stdout_path": str(j.stdout_path),
            "stderr_path": str(j.stderr_path),
        },
    )


@app.get("/logs")
async def logs(block_number: str, n: int = 200):
    j = JOBS.get(block_number)
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
            "stdout": tail(j.stdout_path, n),
            "stderr": tail(j.stderr_path, n),
        },
    )
