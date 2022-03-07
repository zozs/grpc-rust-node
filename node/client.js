import * as grpc from '@grpc/grpc-js'
import * as protoLoader from '@grpc/proto-loader'
import { promisify } from 'util'

const packageDefinition = protoLoader.loadSync('../proto/rustnodegrpc.proto')
const { rustnodegrpc } = grpc.loadPackageDefinition(packageDefinition)

// Create client
const client = new rustnodegrpc.Stats('127.0.0.1:9800', grpc.credentials.createInsecure())

// Get mean by talking to server, using a callback
client.mean({ a: 4, b: 5 }, (err, response) => {
  console.log('response:', response)
})

// Get mean by talking to sereither by callback or as promisever, using a promise
const meanFunc = promisify(client.mean).bind(client)

console.log('response:', await meanFunc({ a: 5, b: 4 }))
