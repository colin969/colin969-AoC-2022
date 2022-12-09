import fs from 'fs';

type FileNode = {
  name: string;
  type: 'file' | 'directory';
  size: number;
  children: FileNode[];
  parent?: FileNode;
}

async function run() {
  const input = fs.readFileSync('./day7/input.txt', { encoding: 'utf8' });
  
  const splits = input.split(/\r?\n/);

  let rootNode: FileNode = {
    name: '/',
    size: 0,
    type: 'directory',
    children: [],
  }
  let curNode = rootNode;

  for (const line of splits) {
    if (line.startsWith("$")) {
      // Command
      const parts = line.split(" ");
      switch (parts[1]) {
        case 'cd': {
          const dir = parts[2];
          switch (dir) {
            case '/': {
              curNode = rootNode;
              break;
            }
            case '..': {
              curNode = curNode.parent!;
              break;
            }
            default: {
              const child = curNode.children.find((child) => child.name === dir);
              if (child) {
                curNode = child;
              } else {
                let newChild: FileNode = {
                  name: dir,
                  type: 'directory',
                  size: 0,
                  children: [],
                  parent: curNode
                };
                curNode.children.push(newChild);
                curNode = newChild;
              }
              break;
            }
          }
          break;
        }
        case 'ls': {
          // Ignore
          break;
        }
        default: {
          console.log(`Unknown command: ${parts[1]}`);
          break;
        }
      }
    } else {
      // File Listing
      const [sizeRaw, name] = line.split(" ");
      const size = parseInt(sizeRaw);
      if (!Number.isNaN(size)) {
        const newChild: FileNode = {
          name,
          type: 'file',
          size,
          children: [],
          parent: curNode
        };
        curNode.children.push(newChild);
      }
    }
  }
  calcNodeSize(rootNode);
  printNode(rootNode, 0);
  console.log(sumLargerThan(rootNode, 100000, 0));
  const spaceNeeded = 30000000 - (70000000 - (rootNode.size));
  const viableCandidates = flattenList(rootNode, []).sort((a, b) => b.size - a.size).reverse();
  console.log(`Viable deletion candidates to save ${spaceNeeded} bytes:`);
  console.log(viableCandidates.filter(node => node.size >= spaceNeeded).map((node) => `${node.name} (${node.size})`));
}

function printNode(node: FileNode, indent: number) {
  console.log(`${'-'.repeat(indent)}${node.name} (${node.size}) (${node.type})`);
  for (const child of node.children) {
    printNode(child, indent + 2);
  }
}

function calcNodeSize(node: FileNode) {
  let size = node.size;
  for (const child of node.children) {
    size += calcNodeSize(child);
  }
  node.size = size;
  return size;
}

function sumLargerThan(node: FileNode, limit: number, cur: number) {
  if (node.size <= limit) {
    cur += node.size;
  }
  for (const child of node.children.filter((child) => child.type === 'directory')) {
    cur = sumLargerThan(child, limit, cur);
  }
  return cur;
}

function flattenList(node: FileNode, list: FileNode[]) {
    list.push(node);
  for (const child of node.children.filter((child) => child.type === 'directory')) {
    list = flattenList(child, list);
  }
  return list;
}

run();