# Minecraft log filter
A very simple minecraft log filter.

## Installation
- TBD.

## How to use

### See who is online (`ps`)
```console
cat [LOGFILE] | mclf ps
```
or
```console
mclf ps "$(cat [LOGFILE])"
```

for example, if I got a log like this:
<details>
<summary> example log </summary>
  
```log
2023-08-20T13:48:06.331568924Z [13:48:06] [Server thread/INFO]: ONR ◆XLTU joined the game
2023-08-20T13:54:31.046531707Z [13:54:31] [Server thread/INFO]: ◆jessieOuO0309 joined the game
2023-08-20T13:54:35.797879107Z [13:54:35] [Server thread/INFO]: ◆leo25246306 joined the game
2023-08-20T14:00:46.572340793Z [14:00:46] [Server thread/INFO]: ONR ◆XLTU left the game
2023-08-20T14:00:49.057749168Z [14:00:49] [Server thread/INFO]: ONR ◆XLTU joined the game
2023-08-20T15:54:57.488134636Z [15:54:57] [Server thread/INFO]: ONR ◆XLTU left the game
2023-08-20T15:57:52.513674358Z [15:57:52] [Server thread/INFO]: ONR ◆XLTU joined the game
2023-08-20T17:05:36.084182034Z [17:05:36] [Server thread/INFO]: ◆jessieOuO0309 left the game
2023-08-20T17:49:17.979761058Z [17:49:17] [Server thread/INFO]: ONR ◆XLTU left the game
2023-08-20T18:10:57.475226934Z [18:10:57] [Server thread/INFO]: ◆leo25246306 left the game
2023-08-21T11:07:01.700238287Z [11:07:01] [Server thread/INFO]: ◆leo25246306 joined the game
2023-08-21T11:57:42.699879130Z [11:57:42] [Server thread/INFO]: ◆jessieOuO0309 joined the game
```

</details>

Then this feature will show the result like this:

```
◆leo25246306 is online
◆jessieOuO0309 is online
```

### Show player's chat history
- TBD.

## Why ?
Reading logs from Minecraft server is too hard.

For the sake of my eye health, I decided to write a small tool for the functions I use often. 

Although many people have probably already done similar things, haha.
