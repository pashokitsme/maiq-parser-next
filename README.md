## TL;DR
Было крайне неудобно иметь такое расписание *(ни уведомлений, ни адаптации под мобильные экраны, ни стабильной работы самого сайта)* в шараге, поэтому я сделал бота в телеграме, который умеет его парсить, сравнивать и присылать уведомление об изменении.
![Пример расписания](./screenshots/rsp-example.jpg)
На данный момент поддерживает сразу несколько групп на одного пользователя. \
Уведомления приходят только для изменённых групп.

## Building
Сложности возникают разве что с sqlx и его проверкой sql-запросов. Решается указанием `DATABASE_URL` или `SQLX_OFFLINE 1`.

> Для `maiq-bot` необходимо указать `TELOXIDE_TOKEN` \
> Для `maiq-db` необходимо указать `SQLITE_PATH`. `DATABASE_URL` использует sqlx для проверки запросов и не обязателен для билда, а `SQLITE_PATH` - реальный файл .sqlite

**docker**
```sh
docker build -t maiq-bot .
docker run -v <PATH>:/var/sqlite.db -e RUST_LOG=info -e RUST_LOG_STYLE=always -e SQLITE_PATH=/var/sqlite.db -e TELOXIDE_TOKEN=<token> -dit maiq-bot
```