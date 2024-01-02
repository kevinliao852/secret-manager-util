**Secret Manager Util**

**Description:**
`secret-manager-util` is a simple Rust command-line utility for comparing secrets stored in AWS Secrets Manager. This tool allows you to specify two secret names and optionally a region for comparison. It retrieves the secrets associated with the given names, compares them, and prints the differing keys.

**Usage:**

```bash
secret-manager-util --secret_name <SECRET_NAME> --secret_name_v2 <SECRET_NAME_V2> [--region <REGION>]
```

- `--secret_name`: Specifies the name of the first secret for comparison.
- `--secret_name_v2`: Specifies the name of the second secret for comparison.
- `--region`: (Optional) Specifies the AWS region. Defaults to `ap-northeast-1` if not provided.

**Example:**

```bash
secret-manager-util --secret_name my_secret --secret_name_v2 my_secret_v2 --region us-west-2
```

**Installation:**
To build and run the utility, follow these steps:

1. Clone the repository:

   ```bash
   git clone git@github.com:kevinliao852/secret-manager-util.git
   ```

2. Navigate to the project directory:

   ```bash
   cd secret-manager-util
   ```

3. Build and run the utility:
   ```bash
   cargo run --release
   ```

**Dependencies:**

- `aws_config`: Manages AWS configuration, including the region.
- `aws_sdk_secretsmanager`: Interacts with AWS Secrets Manager to retrieve secret values.
- `clap`: Parses command-line arguments and generates a user-friendly CLI.
- `serde_json`: Parses and manipulates JSON data.
- `tokio`: Provides asynchronous runtime for the main function.

**Contributing:**
Feel free to contribute to the project by opening issues or submitting pull requests. Please follow the contribution guidelines outlined in the repository.

**License:**
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**Author:**

- Kevin Liao (GitHub: [your_username](https://github.com/kevinliao852))
