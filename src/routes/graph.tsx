import { createFileRoute } from "@tanstack/react-router";

import { z } from "zod";
import { zodValidator } from "@tanstack/zod-adapter";
import { projectSourceSchema, projectTypeSchema } from "../schemas";

const graphParamsSchema = z.object({
    source: projectSourceSchema,
    value: z.string(),
    type: projectTypeSchema
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
