class IR:
def __init__(self, nodes):
self.nodes = nodes


def serialize(self) -> bytes:
return '\n'.join(self.nodes).encode()