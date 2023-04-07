import { GreeterClient, HelloRequest, HelloReply } from './grpc/helloworld_grpc_web_pb.js';

let grpcHost = new URL(window.location.href);
grpcHost.pathname = "";
grpcHost.query = "";

var greeterService = new GreeterClient(grpcHost.toString());

var request = new HelloRequest();
request.setName('friend');

greeterService.sayHello(request, {}, function(err, response) {
  document.write("<p>" + response.getMessage() + "</p>");
});
