import esgit from './es-git.mjs';

const { cloneRepository } = esgit;

const [, , path] = process.argv;

const repo = await cloneRepository('https://github.com/toss/es-toolkit', path);
const commit = repo.getCommit('b8da6946cfc2d1ec8f10637732a26a579c7c8b82');
const entry = commit.tree().getName('README.md');
const blob = entry.toObject(repo).peelToBlob();

const content = new TextDecoder().decode(blob.content());
console.log(content);
