# wcsv
A tool written in Rust to generate a CSV of info about all the countries in the world
# Usage
```
cargo run
```
It'll ask you for an API key, you can get that from [API Ninjas](https://api-ninjas.com/) <br/>
You'll have an `apikey` file in the directory, don't worry this is normal, then it'll fetch all the countries (this may take some time depending on your network speed) and then it'll arrange them in a CSV called `world_countries_info.csv`.