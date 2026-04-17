"""HTTP client for the Rust verifier service."""

import json
import urllib.error
import urllib.request
from typing import Any, Optional


class RustVerifierClientError(RuntimeError):
    """Raised when the verifier service cannot complete a request."""


class RustVerifierClient:
    """Small standard-library client for the verifier HTTP API."""

    def __init__(self, base_url: str, timeout: float = 10.0):
        self.base_url = base_url.rstrip("/")
        self.timeout = timeout

    def verify(
        self,
        task: dict[str, Any],
        response: str,
        domain: str,
        verifier: Optional[str] = None,
        task_id: Optional[str] = None,
    ) -> dict[str, Any]:
        payload = {
            "domain": domain,
            "task": task,
            "response": response,
        }
        if verifier:
            payload["verifier"] = verifier
        if task_id:
            payload["task_id"] = task_id
        return self._post("/verify", payload)

    def verify_batch(self, requests: list[dict[str, Any]]) -> list[dict[str, Any]]:
        return self._post("/verify/batch", requests)

    def health(self) -> dict[str, Any]:
        req = urllib.request.Request(f"{self.base_url}/health", method="GET")
        try:
            with urllib.request.urlopen(req, timeout=self.timeout) as resp:
                return json.loads(resp.read().decode("utf-8"))
        except urllib.error.URLError as exc:
            raise RustVerifierClientError(str(exc)) from exc

    def _post(self, path: str, payload: Any) -> Any:
        data = json.dumps(payload).encode("utf-8")
        req = urllib.request.Request(
            f"{self.base_url}{path}",
            data=data,
            headers={"content-type": "application/json"},
            method="POST",
        )
        try:
            with urllib.request.urlopen(req, timeout=self.timeout) as resp:
                return json.loads(resp.read().decode("utf-8"))
        except urllib.error.HTTPError as exc:
            body = exc.read().decode("utf-8", errors="replace")
            raise RustVerifierClientError(f"HTTP {exc.code}: {body}") from exc
        except urllib.error.URLError as exc:
            raise RustVerifierClientError(str(exc)) from exc
