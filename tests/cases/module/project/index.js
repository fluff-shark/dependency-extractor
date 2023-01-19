import { print } from "./a/print.js"
import { printList } from "./b/print-list.js"
import { Buffer } from 'node:buffer';

printList(["a", "b"]);
print(Buffer.allocUnsafe(10).toString('utf8'));
