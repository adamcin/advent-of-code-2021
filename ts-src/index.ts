const internalHelloWorld = (): string => 'hello, world!';
console.log(internalHelloWorld());
export const getHelloWorld = internalHelloWorld;