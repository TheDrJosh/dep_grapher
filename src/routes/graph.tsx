import { createFileRoute } from "@tanstack/react-router";

import { z } from "zod";
import { zodValidator } from "@tanstack/zod-adapter";

const graphParamsSchema = z.object({
    source: z.enum(["local", "git", "cargo", "npm", "jsr", "pypi"]),
    value: z.string(),
});

export type PackageInfo = z.infer<typeof graphParamsSchema>;
export type PackageSource = PackageInfo["source"];

export const Route = createFileRoute("/graph")({
    component: RouteComponent,
    validateSearch: zodValidator(graphParamsSchema),
});

function RouteComponent() {
    const { source, value } = Route.useSearch();

    return (
        <div>
            <span>source: {source}</span>
            <span>value: {value}</span>
        </div>
    );
}
