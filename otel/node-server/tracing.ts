/*instrumentation.ts*/
import { NodeSDK } from "@opentelemetry/sdk-node";
import { OTLPTraceExporter } from "@opentelemetry/exporter-trace-otlp-grpc";
import { Resource } from "@opentelemetry/resources";
import { SemanticResourceAttributes } from "@opentelemetry/semantic-conventions";
import { propagation, context } from "@opentelemetry/api";
import { Request } from "express";

const sdk = new NodeSDK({
  resource: new Resource({
    [SemanticResourceAttributes.SERVICE_NAME]: "Roll Service",
    [SemanticResourceAttributes.SERVICE_VERSION]: "1.0",
  }),
  traceExporter: new OTLPTraceExporter({
    url: "http://tempo:4317",
  }),
});

export function startSdk() {
  sdk.start();
}

export function extractContext(req: Request) {
  let ctx = propagation.extract(context.active(), req.headers);
  return ctx;
}
