// Generated by ts-to-zod
import { z } from "zod";

export const gitCommitSchema = z.union([
    z.object({
        commit: z.string(),
    }),
    z.object({
        tag: z.string(),
    }),
]);

export const gitPackageTypeSchema = z.union([
    z.object({
        local: z.string(),
    }),
    z.object({
        remote: z.string(),
    }),
]);

export const gitResolveErrorSchema = z.never();

export const invalidPathErrorSchema = z.union([
    z.literal("not_found"),
    z.literal("not_a_directory"),
    z.literal("invalid_name"),
    z.literal("not_absolute"),
    z.literal("unknown"),
]);

export const languageSchema = z.union([
    z.literal("rust"),
    z.literal("node_js"),
    z.literal("deno"),
    z.literal("python"),
    z.literal("zig"),
]);

export const localPackageLocationSchema = z.object({
    path: z.string(),
});

export const localResolveErrorSchema = z.union([
    z.literal("no_packages_in_path"),
    z.object({
        invalid_path: invalidPathErrorSchema,
    }),
]);

export const gitPackageLocationSchema = z.object({
    git_type: gitPackageTypeSchema,
    commit: gitCommitSchema,
});

export const registryTypeSchema = z.union([
    z.literal("cargo"),
    z.literal("npm"),
    z.literal("jsr"),
    z.literal("pypi"),
]);

export const registrySchema = z.object({
    registry_type: registryTypeSchema,
    custom_url: z.string().nullable(),
});

export const registryResolveErrorSchema = z.never();

export const resolveErrorSchema = z.union([
    z.object({
        local: localResolveErrorSchema,
    }),
    z.object({
        git: gitResolveErrorSchema,
    }),
    z.object({
        registry: registryResolveErrorSchema,
    }),
]);

export const searchRegistryErrorSchema = z.union([
    z.literal("network"),
    z.literal("server"),
    z.literal("parse"),
]);

export const searchResultSchema = z.object({
    name: z.string(),
});

export const registryPackageLocationSchema = z.object({
    registry: registrySchema,
    name: z.string(),
    version: z.string(),
});

export const packageLocationSchema = z.union([
    z.object({
        local: localPackageLocationSchema,
    }),
    z.object({
        git: gitPackageLocationSchema,
    }),
    z.object({
        registry: registryPackageLocationSchema,
    }),
    z.object({
        url: z.string(),
    }),
    z.object({
        unknown: z.object({
            name: z.string(),
            description: z.string(),
        }),
    }),
]);

export const packageSchema = z.object({
    location: packageLocationSchema,
    name: z.string(),
    version: z.string(),
    description: z.string().nullable(),
    authors: z.array(z.string()),
    language: languageSchema,
    dependencies: z.array(packageLocationSchema),
});
