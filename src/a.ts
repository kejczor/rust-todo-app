function ID() {
  let id = 1;
  return () => ++id;
}

const idGenerator = ID();
console.log(idGenerator());
console.log(idGenerator());
console.log(idGenerator());
