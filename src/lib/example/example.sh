killall private-id-server &
killall private-id-client &

# Company
cargo run --bin private-id-server -- \
  --host '0.0.0.0:3001' --no-tls \
  --use-row-numbers \
  --input example/email_company.csv --stdout \
  &

sleep 0.5;

# Partner
cargo run --bin private-id-client -- \
  --company 'localhost:3001' --no-tls \
  --use-row-numbers \
  --input example/email_partner.csv --stdout;
