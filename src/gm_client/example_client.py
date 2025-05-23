import grpc
from .grpc_server_pb2 import ServerLiveRequest, ServerReadyRequest
from .grpc_server_pb2_grpc import PredictionServiceStub

class Client:
    """
    Python wrapper for the PredictionService gRPC API.
    """

    def __init__(self, target: str = "localhost:50051", secure: bool = False, **kwargs):
        """
        :param target: gRPC server address (e.g. "localhost:50051")
        :param secure: if True, uses a secure channel (requires certificates)
        :param kwargs: extra options passed to grpc.channel
        """
        if secure:
            # Example: load certificate files here
            # creds = grpc.ssl_channel_credentials(...)
            # self._channel = grpc.secure_channel(target, creds, **kwargs)
            raise NotImplementedError("Secure channel not yet implemented")
        else:
            self._channel = grpc.insecure_channel(target, **kwargs)

        self._stub = PredictionServiceStub(self._channel)

    def server_live(self) -> str:
        """
        Calls the server_live RPC and returns its message.
        """
        req = ServerLiveRequest()
        resp = self._stub.server_live(req)
        return resp.message

    def server_ready(self) -> str:
        """
        Calls the server_ready RPC and returns its message.
        """
        req = ServerReadyRequest()
        resp = self._stub.server_ready(req)
        return resp.message

    def close(self):
        """Shut down the gRPC channel."""
        self._channel.close()

