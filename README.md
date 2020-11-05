## Summarize telegram group messages 

This is source code of a [Telegram](https://telegram.org/) bot written in Rust, that can summarize the group chat messages. Useful if you often find yourself in the situation where there are too many unread messages in the group chat you're part of.

https://telegramic.org/bot/summarizatorbot/

### How it works

1. Start conversation with the bot.
2. Add bot to the group chat. 
3. Send `/digest` command to the bot
4. Expect a personal message from the bot with the messages digest

### Boot up locally

1. In the root folder create `.env` file with this content

```
TG_TOKEN=<your_tg_token>
DB_HOST=<dgraph_host, ie. localhost:9800>
```

2. Run the `dgraph` database on your machine

3. To start the python word summarizator  
`cd src/telegram/text_analysis`  
`export FLASK_APP=main.py`  
`python3 -m flask run`  

4. Start the main app with  
`cargo run`
