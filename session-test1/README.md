https://lib.rs/crates/actix-session

test via curl 

saves the cookies received in the response to a file named cookies.txt. The -i option tells curl to include the HTTP response headers in the output.

```
 ✗ curl -c cookies.txt -i http://localhost:8080
HTTP/1.1 200 OK
content-length: 11
set-cookie: id=agkjTCixOCe0VTiN2FsbI2eQnHjLYfYQR8FHepdL8x0zrc0S%2FM1pqPrNVw%3D%3D; HttpOnly; SameSite=Lax; Path=/
date: Tue, 16 Apr 2024 04:46:58 GMT

Count is 1!%  
```

includes the cookies from cookies.txt in the request. The -b option tells curl to read cookies from the specified file.


```                                                                                          
✗ curl -b cookies.txt -i http://localhost:8080
HTTP/1.1 200 OK
content-length: 11
set-cookie: id=mcc2rx1OfiLVb4uH4mkEncKSiDjjZ0N77CpN%2FJR2EsJK43Nq5cozlguXYA%3D%3D; HttpOnly; SameSite=Lax; Path=/
date: Tue, 16 Apr 2024 04:47:01 GMT

Count is 2!% 
```
