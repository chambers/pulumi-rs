# Requirements

To bring up a new language for Pulumi - the following is required:

- [P0] Create a language host (e.g. 

  https://github.com/pulumi/pulumi/blob/master/sdk/dotnet/cmd/pulumi-language-dotnet/main.go

  )

  - A plugin that serves the [language.proto](https://github.com/pulumi/pulumi/blob/master/sdk/proto/language.proto) gRPC API and invokes programs in the target language

- [P0] Create a Pulumi SDK that is feature-parity with the  `@pulumi/pulumi`SDK in https://github.com/pulumi/pulumi/tree/master/sdk

  - The SDK will internally be a gRPC client for the [resource.proto](https://github.com/pulumi/pulumi/blob/master/sdk/proto/resource.proto) gRPC API which connects to the Pulumi engine gRPC server provided by the language host above, and turns `new Resource` calls in the host language into `RegisterResource` gRPC calls to the Pulumi engine.

- [P1] Add codegen support to 

  https://github.com/pulumi/pulumi/tree/master/pkg/codegen

  - Codegen support enables generating SDKs in the new language for any Pulumi Packages (including resource providers)

- [P1] Wire codegen for the new language into `tfgen` https://github.com/pulumi/pulumi-terraform-bridge/tree/master/pkg/tfgen and other package SDK generators (https://github.com/pulumi/pulumi-kubernetes/tree/master/cmd/pulumi-gen-kubernetes, https://github.com/pulumi/pulumi-azure-native/blob/master/provider/cmd/pulumi-gen-azure-native, etc.)

- [P2] Add package publishing automation for the language's package manager and get that adopted into all packages

- [P2] Add support to integration test harness in https://github.com/pulumi/pulumi/blob/master/pkg/testing/integration/program.go

- [P3] 10 Examples in https://github.com/pulumi/examples

- [P3] Langauge docs in https://www.pulumi.com/docs/index.html

- [P4] All langchoose items in docs support the new language - in particular everything in https://www.pulumi.com/docs/intro/concepts/programming-model/.

- [P4] Templates added to https://github.com/pulumi/templates.

- [P5] Automation API

- [P5] Add support for the new language to [`tf2pulumi`](https://github.com/pulumi/tf2pulumi)
