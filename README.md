# CAGrepeat-analyzer
- plot CAG reapeats human genome.
- implemented a tokenizer approach with async to enable faster searches as compared to regex based.
- profiling all CAG repeat across human genome or any other genome at async speed.
- make sure you have matplotlib installed.

```
____      _       ____   ____                                  _
/ ___|    / \     / ___| |  _ \    ___   _ __     ___    __ _  | |_
| |       / _ \   | |  _  | |_) |  / _ \ | '_ \   / _ \  / _` | | __|
| |___   / ___ \  | |_| | |  _ <  |  __/ | |_) | |  __/ | (_| | | |_
\____| /_/   \_\  \____| |_| \_\  \___| | .__/   \___|  \__,_|  \__|
                                       |_|

CAG repeat pattern.
       ************************************************
       Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************

Usage: CAGrepeat-analyzer <COMMAND>

Commands:
  cag-repeat  CAG repeat pattern
  cag-plot    plot the CAG
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
./target/debug/CAGrepeat-analyzer cag-repeat ./test-files/test.fa
ENST00000832824.1	4
./target/debug/CAGrepeat-analyzer cag-plot ./test-files/test.fa outputnew
```

Gaurav Sablok
codeprog@icloud.com
