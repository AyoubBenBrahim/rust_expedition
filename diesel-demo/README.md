

following the diesel getting started tuto

https://diesel.rs/guides/getting-started

CRUDs via diesel

some usful commands:

doc exec -it diesel-demo psql -U diesel -c "\d posts" 

cargo run --bin show_posts

doc exec -it diesel-demo psql -U diesel -c "select * from posts"
