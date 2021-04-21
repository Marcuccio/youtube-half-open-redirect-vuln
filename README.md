# How to run
`cargo run --quiet`

# Build
`cargo build --release`

found in [youtube](https://www.youtube.com/watch?v=UTMlT-Xr8Is) 
Read about the vulnerability [here](https://www.untrustednetwork.net/en/2019/07/22/half-open-redirect-vulnerability-in-youtube/)

### Malicious redirect

https://www.youtube.com/redirect?event=comments&redir_token=QUFFLUhqbjk1bkxnamRJSXA1ZHFSZlIzdkQ4RWZWekIzUXxBQ3Jtc0tudHloUlNaM3FxS1hTbGl5M1Q5SGpiNmI3MkNSbU5fSnVNNTBHOTh6b05meVNEeFcwMENpNnEtd3ZKYmZxTGYzR2tnd04tRWpRZnhMQ1I4eHJWNG1YT1BrUlFLbE96Nmg5bHdzcy1OOGVBQ2ROM05KTQ&q=http%3A%2F%2Fvon.fyi%2F&stzid=UgwwMzrz7n-x8CirS214AaABAg

http://www.claudiogiunta.it/

### Malicious redirect decoded

https://www.youtube.com/redirect?event=comments&redir_token=QUFFLUhqbjk1bkxnamRJSXA1ZHFSZlIzdkQ4RWZWekIzUXxBQ3Jtc0tudHloUlNaM3FxS1hTbGl5M1Q5SGpiNmI3MkNSbU5fSnVNNTBHOTh6b05meVNEeFcwMENpNnEtd3ZKYmZxTGYzR2tnd04tRWpRZnhMQ1I4eHJWNG1YT1BrUlFLbE96Nmg5bHdzcy1OOGVBQ2ROM05KTQ&q=http://von.fyi/&stzid=UgwwMzrz7n-x8CirS214AaABAg

### Malicious URL parameters
```
event=comments
redir_token=QUFFLUhqbjk1bkxnamRJSXA1ZHFSZlIzdkQ4RWZWekIzUXxBQ3Jtc0tudHloUlNaM3FxS1hTbGl5M1Q5SGpiNmI3MkNSbU5fSnVNNTBHOTh6b05meVNEeFcwMENpNnEtd3ZKYmZxTGYzR2tnd04tRWpRZnhMQ1I4eHJWNG1YT1BrUlFLbE96Nmg5bHdzcy1OOGVBQ2ROM05KTQ
q=http://von.fyi/
stzid=UgwwMzrz7n-x8CirS214AaABAg
```
----------------------

Safe redirect

https://www.youtube.com/redirect?event=comments&redir_token=QUFFLUhqbjk1bkxnamRJSXA1ZHFSZlIzdkQ4RWZWekIzUXxBQ3Jtc0tudHloUlNaM3FxS1hTbGl5M1Q5SGpiNmI3MkNSbU5fSnVNNTBHOTh6b05meVNEeFcwMENpNnEtd3ZKYmZxTGYzR2tnd04tRWpRZnhMQ1I4eHJWNG1YT1BrUlFLbE96Nmg5bHdzcy1OOGVBQ2ROM05KTQ&q=http://www.claudiogiunta.it/
