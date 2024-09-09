import grpc from 'k6/net/grpc';
import { check, sleep } from 'k6';

const connectArgs = {
  plaintext: false,
  tls: {
    cacerts: [open('../../certs/root_server_cert.pem')],
    cert: open('../../certs/client_cert.pem'),
    key: open('../../certs/client_privkey.pem'),
  }
};

const client = new grpc.Client();
client.load(['./proto'], 'app.proto');

export default () => {
  client.connect('localhost:50051', connectArgs);

  const req = { name: 'key1' };
  const resp = client.invoke('app.App/get', req);

  check(resp, {
    'status is OK': (r) => r && r.status === grpc.StatusOK,
  });

  console.log(JSON.stringify(resp.message));

  client.close();
  sleep(1);
}
