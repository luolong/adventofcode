import { TextLineStream } from "jsr:@std/streams/text-line-stream";

export async function readLines(path: string): Promise<string[]> {
  using f = await Deno.open(path);
  const readable = f.readable
    .pipeThrough(new TextDecoderStream()) // decode Uint8Array to string
    .pipeThrough(new TextLineStream()); // split string line by line

  return await Array.fromAsync(readable);
}
