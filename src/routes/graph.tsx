import { createFileRoute } from "@tanstack/react-router";

import { z } from "zod";
import { zodValidator } from "@tanstack/zod-adapter";

const graphParamsSchema = z.object({
    type: z.enum(["local", "git", "cargo", "npm", "jsr", "pip"]),
    path: z.string(),
});

export type GraphParams = z.infer<typeof graphParamsSchema>;

export const Route = createFileRoute("/graph")({
    component: RouteComponent,
    validateSearch: zodValidator(graphParamsSchema),
});

function RouteComponent() {
    const { type, path } = Route.useSearch();

    return (
        <div>
            <span>type: {type}</span>
            <span>path: {path}</span>
        </div>
    );
}
