import { $ } from "bun";

type Counter = { count: number; covered: number };
type FileReport = { filename: string; summary: Record<"branches" | "regions", Counter> };

const ignore = "(_test\\.rs|/tests/)";
const raw = await $`cargo llvm-cov --branch --ignore-filename-regex ${ignore} --json --summary-only`.text();
const files: FileReport[] = JSON.parse(raw).data[0].files;

for (const metric of ["branches", "regions"] as const) {
  const gaps = files.filter((f) => f.summary[metric].covered < f.summary[metric].count);
  if (gaps.length > 0) {
    for (const f of gaps) {
      const c = f.summary[metric];
      console.error(`${metric} ${c.covered}/${c.count} ${f.filename}`);
    }
    await $`cargo llvm-cov report --branch --ignore-filename-regex ${ignore} --show-missing-lines`.nothrow();
    process.exit(1);
  }
  const total = files.reduce((n, f) => n + f.summary[metric].count, 0);
  console.log(`${metric} 100% (${total})`);
}
