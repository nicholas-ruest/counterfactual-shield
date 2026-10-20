#!/usr/bin/env node
import { execFileSync } from "node:child_process";

const raw = execFileSync("cargo", ["metadata", "--format-version", "1", "--locked"], {
  encoding: "utf8", maxBuffer: 50 * 1024 * 1024,
});
const metadata = JSON.parse(raw);
const local = metadata.packages.filter((pkg) => pkg.source === null && pkg.name.startsWith("cs-"));
const localIds = new Set(local.map((pkg) => pkg.id));
const incoming = new Map(local.map((pkg) => [pkg.id, []]));

for (const node of metadata.resolve.nodes) {
  if (!localIds.has(node.id)) continue;
  for (const dependency of node.dependencies) {
    if (localIds.has(dependency)) incoming.get(dependency).push(node.id);
  }
}

const violations = [];
for (const pkg of local) {
  const hasBinary = pkg.targets.some((target) => target.kind.includes("bin"));
  const dependents = incoming.get(pkg.id);
  if (!hasBinary && dependents.length === 0) {
    violations.push(pkg.name);
  }
  if (pkg.targets.every((target) => target.src_path.endsWith("lib.rs")) &&
      pkg.targets.every((target) => target.kind.includes("lib")) &&
      !pkg.targets.some((target) => target.test)) {
    violations.push(`${pkg.name}:no-test-target`);
  }
}

const report = {
  local_crates: local.length,
  dependency_edges: [...incoming.values()].reduce((sum, values) => sum + values.length, 0),
  executable_crates: local.filter((pkg) => pkg.targets.some((target) => target.kind.includes("bin"))).map((pkg) => pkg.name),
  crates_without_callers_or_entrypoint: violations,
};
console.log(JSON.stringify(report, null, 2));
if (violations.length > 0) process.exit(1);
