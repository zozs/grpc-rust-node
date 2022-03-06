import * as grpc from '@grpc/grpc-js'
import * as protoLoader from '@grpc/proto-loader'

const packageDefinition = protoLoader.loadSync('../proto/rustnodegrpc.proto')
const { rustnodegrpc } = grpc.loadPackageDefinition(packageDefinition)

// Implemented function
function mean(call, callback) {
  const { a, b } = call.request
  console.log('got request:', call.request)
  callback(null, { mean: (a + b) / 2 })
}

// Launch service
const server = new grpc.Server()

// load service Stats from proto file
server.addService(rustnodegrpc.Stats.service, { mean })
server.bindAsync('127.0.0.1:9800', grpc.ServerCredentials.createInsecure(), () => {
  server.start()
})
