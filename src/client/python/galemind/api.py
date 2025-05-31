import grpc
from galemind.pb.prediction_pb2_grpc import PredictionServiceStub  
from pydantic import BaseModel

class ClientContext(BaseModel):
    certificate_path: str
    ssl_enable: bool
    host: str
    port: int

class GaleMindClient:
    def __init__(self, service: PredictionServiceStub):
        self._service = service

    @classmethod
    def create(cls, ctx: 'ClientContext') -> 'GaleMindClient':
        if ctx.ssl_enable:
            with open(ctx.certificate_path, 'rb') as f:
                trusted_certs = f.read()
            credentials = grpc.ssl_channel_credentials(root_certificates=trusted_certs)
            channel = grpc.secure_channel(f"{ctx.host}:{ctx.port}", credentials)
        else:
            channel = grpc.insecure_channel(f"{ctx.host}:{ctx.port}")

        service = PredictionServiceStub(channel)
        return cls(service)