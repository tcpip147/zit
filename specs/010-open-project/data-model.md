# Data Model: Open Project

## OpenProjectForm

Temporary UI state owned by the application shell.

| Field | Type | Rules |
|---|---|---|
| project_name | text | Required after trimming; becomes the tab name |
| repository_path | text | Required; existing directory containing `.git` |
| error | optional text | Recoverable validation or chooser message |

## ProjectMetadata

Existing persisted entity containing identity, repository path, display name, and availability. A successful submission creates one page and metadata entry, activates it, and persists the session.

## State transitions

`Closed -> OpenForm` on File > Open; `OpenForm -> OpenForm with error` on invalid confirmation; `OpenForm -> Closed + ActiveProject` on valid confirmation; `OpenForm -> Closed` on Cancel/Escape.
