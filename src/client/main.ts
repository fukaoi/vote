/**
 * Hello world
 */

import {
  callVote,
  establishConnection,
  establishPayer,
  loadProgram,
} from './vote';

async function main() {
  console.log("---- establishConnection ----");
  await establishConnection();

  console.log("---- establishPayer      ----");
  await establishPayer();

  console.log("---- loadProgram         ----");
  await loadProgram();

  console.log("---- callVote            ----");
  await callVote();
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
