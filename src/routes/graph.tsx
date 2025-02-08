import { createFileRoute } from "@tanstack/react-router";

import { zodValidator } from "@tanstack/zod-adapter";
import { packageLocationSchema } from "../bindings.zod";


export const Route = createFileRoute("/graph")({
    component: RouteComponent,
    validateSearch: zodValidator(packageLocationSchema),
});

function RouteComponent() {
    const package_location = Route.useSearch();

    return (
        <div>
            <span>location: {JSON.stringify(package_location)}</span>
        </div>
    );
}
