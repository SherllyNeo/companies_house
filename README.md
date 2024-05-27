# companies_house
A simple companies house API interface

This is for teaching purposes.

Heavily commented and not always using best practices for flexible code/custom error handling.

## Quick Start
1. Go to here [https://developer.company-information.service.gov.uk/manage-applications/](https://developer.company-information.service.gov.uk/manage-applications/) 

2. Make an application and put the API key in the .env file, then 
```bash
source .env
cargo build --release
```
3. Run the program, input the name of a company and it will return the top ----------
```bash
./target/release/companies_house
```

4. Learn from the source code


