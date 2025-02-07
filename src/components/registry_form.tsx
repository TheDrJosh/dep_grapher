import {
    type FieldApi,
    type ReactFormExtendedApi,
    useForm,
} from "@tanstack/react-form";
import { Command } from "cmdk";
import { Search } from "lucide-react";
import { commands, type RegistryType } from "../bindings";
import { useQuery } from "@tanstack/react-query";

interface FormValues {
    name: string;
    version: string;
}

export function RegistryProjectForm(props: { registry_type: RegistryType }) {
    const { registry_type } = props;
    const form = useForm<FormValues>({
        defaultValues: {
            name: "",
            version: "",
        },
    });

    return (
        <form
            onSubmit={(e) => {
                e.preventDefault();
                e.stopPropagation();
                form.handleSubmit();
            }}
        >
            <div className="h-48">
                <RegistrySearch form={form} registry_type={registry_type} />
            </div>
            <div>{/* <VersionSelect form={form} /> */}</div>
            <button type="submit">Submit</button>
        </form>
    );
}

function RegistrySearch(props: {
    form: ReactFormExtendedApi<FormValues, undefined>;
    registry_type: RegistryType;
}) {
    const { form, registry_type } = props;

    return (
        <>
            <form.Field name="name">
                {(field) => (
                    <Command
                        label="Search"
                        className="group flex w-full flex-col overflow-hidden rounded-md bg-backround dark:bg-backround-dark"
                    >
                        <div
                            className="peer flex items-center border-b px-3"
                            // eslint-disable-next-line react/no-unknown-property
                            cmdk-input-wrapper=""
                        >
                            <Search className="mr-2 h-4 w-4 shrink-0 opacity-50" />
                            <Command.Input
                                className="flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-neutral-500 disabled:cursor-not-allowed disabled:opacity-50"
                                value={field.state.value}
                                onValueChange={(v) => field.handleChange(v)}
                                onBlur={field.handleBlur}
                            />
                        </div>
                        <Command.List className="max-h-32 overflow-x-hidden overflow-y-auto not-group-focus-within:hidden">
                            <SearchResults
                                field={field}
                                registry_type={registry_type}
                            />
                        </Command.List>
                    </Command>
                )}
            </form.Field>
        </>
    );
}

function SearchResults(props: {
    field: FieldApi<FormValues, "name", undefined, undefined, string>;
    registry_type: RegistryType;
}) {
    const { field, registry_type } = props;

    const { data } = useQuery({
        queryKey: ["registry_search", field.state.value, registry_type],
        queryFn: async () => {
            const res = await commands.searchRegistry(
                {
                    custom_url: null,
                    registry_type: registry_type,
                },
                field.state.value
            );

            if (res.status == "ok") {
                return res.data;
            } else {
                throw res.error;
            }
        },
        staleTime: 300000,
    });

    return (
        <>
            {data ? (
                <>
                    <Command.Empty className="p-6 text-sm">
                        No Results Found
                    </Command.Empty>
                    {data.map((result) => (
                        <Command.Item
                            className="relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-none select-none data-[selected='true']:bg-accent"
                            key={result.name}
                            onSelect={() => field.handleChange(result.name)}
                        >
                            {result.name}
                        </Command.Item>
                    ))}
                </>
            ) : (
                <Command.Empty className="p-6 text-sm">
                    Loading...
                </Command.Empty>
            )}
        </>
    );
}

// function VersionSelect(props: {
//     form: ReactFormExtendedApi<FormValues, undefined>;
// }) {
//     return <></>;
// }
