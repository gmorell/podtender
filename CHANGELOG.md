# v0.5.0:
### Added: -

### Changed:
* support podman version 5.0.0
* update dependencies
* fmt / clean warnings from Df


# v0.5.0:
### Added: -

### Changed:
* support podman version 4.5.0
* update dependencies

### Removed: -

# v0.4.0:
### Added: -

### Changed:
* support podman version 4.4.0
* update dependencies

### Removed: -

# v0.3.0:
### Added: -

### Changed:
* support podman version 4.2.0
* **breaking:** fix pods `stats(...)` method
* add `PodStatsResponse`
* update dependencies

### Removed: -

# v0.2.0:
### Added: -

### Changed: 
* support podman version 4.1.0 
* **breaking:** `GetInfoResponse` and `HostInfo` can't derive `Eq` anymore since a podman API change introduced floats as return types for some fields.
* **breaking:** rename `enable-tracing` feature to `tracing`
* **breaking:** adopt rust version 1.60.0 features syntax 
* update dependencies

### Removed: -

# v0.1.0:
### Added:
* initial release

### Changed: -

### Removed: -