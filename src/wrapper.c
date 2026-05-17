#include <libconfig.h>
#include <stdlib.h>

// ── Config lifecycle wrappers (handles opaque allocation) ──

config_t *wrapper_config_new(void) {
    config_t *config = (config_t *)malloc(sizeof(config_t));
    if (config) {
        config_init(config);
    }
    return config;
}

void wrapper_config_free(config_t *config) {
    if (config) {
        config_destroy(config);
        free(config);
    }
}

// ── Config error macro wrappers ──

const char *wrapper_config_error_text(const config_t *config) {
    return config_error_text(config);
}

const char *wrapper_config_error_file(const config_t *config) {
#if (LIBCONFIG_VER_MAJOR > 1) || (LIBCONFIG_VER_MAJOR == 1 && LIBCONFIG_VER_MINOR >= 4)
    return config_error_file(config);
#else
    return NULL;
#endif
}

int wrapper_config_error_line(const config_t *config) {
    return config_error_line(config);
}

int wrapper_config_error_type(const config_t *config) {
#if (LIBCONFIG_VER_MAJOR > 1) || (LIBCONFIG_VER_MAJOR == 1 && LIBCONFIG_VER_MINOR >= 4)
    return config_error_type(config);
#else
    return 0;
#endif
}

// ── Config root setting macro wrapper ──

config_setting_t *wrapper_config_root_setting(const config_t *config) {
    return config_root_setting(config);
}

// ── Config get_default_format macro wrapper ──

unsigned short wrapper_config_get_default_format(const config_t *config) {
#if (LIBCONFIG_VER_MAJOR > 1) || (LIBCONFIG_VER_MAJOR == 1 && LIBCONFIG_VER_MINOR >= 4)
    return config_get_default_format(config);
#else
    return 0;
#endif
}

// ── Config set_default_format macro wrapper ──

void wrapper_config_set_default_format(config_t *config, unsigned short format) {
#if (LIBCONFIG_VER_MAJOR > 1) || (LIBCONFIG_VER_MAJOR == 1 && LIBCONFIG_VER_MINOR >= 4)
    config_set_default_format(config, format);
#else
    (void)config;
    (void)format;
#endif
}

// ── Config set_auto_convert macro wrapper ──

void wrapper_config_set_auto_convert(config_t *config, int flag) {
    config_set_auto_convert(config, flag);
}

// ── Config get_auto_convert macro wrapper ──

int wrapper_config_get_auto_convert(const config_t *config) {
    return config_get_auto_convert(config);
}

// ── Config get_tab_width macro wrapper ──

unsigned short wrapper_config_get_tab_width(const config_t *config) {
#if (LIBCONFIG_VER_MAJOR > 1) || (LIBCONFIG_VER_MAJOR == 1 && LIBCONFIG_VER_MINOR >= 8)
    return config_get_tab_width(config);
#else
    return 2;
#endif
}

// ── Config get_float_precision macro wrapper ──

unsigned short wrapper_config_get_float_precision(const config_t *config) {
#if (LIBCONFIG_VER_MAJOR > 1) || (LIBCONFIG_VER_MAJOR == 1 && LIBCONFIG_VER_MINOR >= 8)
    return config_get_float_precision(config);
#else
    return 6;
#endif
}

// ── Setting type/name/parent/is_root macros ──

int wrapper_config_setting_type(const config_setting_t *setting) {
    return config_setting_type(setting);
}

const char *wrapper_config_setting_name(const config_setting_t *setting) {
    return config_setting_name(setting);
}

config_setting_t *wrapper_config_setting_parent(const config_setting_t *setting) {
    return config_setting_parent(setting);
}

int wrapper_config_setting_is_root(const config_setting_t *setting) {
    return config_setting_is_root(setting);
}

unsigned int wrapper_config_setting_source_line(const config_setting_t *setting) {
    return config_setting_source_line(setting);
}

const char *wrapper_config_setting_source_file(const config_setting_t *setting) {
#if (LIBCONFIG_VER_MAJOR > 1) || (LIBCONFIG_VER_MAJOR == 1 && LIBCONFIG_VER_MINOR >= 4)
    return config_setting_source_file(setting);
#else
    return NULL;
#endif
}

// ── Setting type check macros ──

int wrapper_config_setting_is_group(const config_setting_t *setting) {
    return config_setting_is_group(setting);
}

int wrapper_config_setting_is_array(const config_setting_t *setting) {
    return config_setting_is_array(setting);
}

int wrapper_config_setting_is_list(const config_setting_t *setting) {
    return config_setting_is_list(setting);
}

int wrapper_config_setting_is_number(const config_setting_t *setting) {
    return config_setting_is_number(setting);
}

int wrapper_config_setting_is_scalar(const config_setting_t *setting) {
#if (LIBCONFIG_VER_MAJOR > 1) || (LIBCONFIG_VER_MAJOR == 1 && LIBCONFIG_VER_MINOR >= 8)
    return config_setting_is_scalar(setting);
#else
    int t = config_setting_type(setting);
    return t == CONFIG_TYPE_INT || t == CONFIG_TYPE_INT64
        || t == CONFIG_TYPE_FLOAT || t == CONFIG_TYPE_STRING
        || t == CONFIG_TYPE_BOOL;
#endif
}

int wrapper_config_setting_is_aggregate(const config_setting_t *setting) {
#if (LIBCONFIG_VER_MAJOR > 1) || (LIBCONFIG_VER_MAJOR == 1 && LIBCONFIG_VER_MINOR >= 8)
    return config_setting_is_aggregate(setting);
#else
    int t = config_setting_type(setting);
    return t == CONFIG_TYPE_GROUP || t == CONFIG_TYPE_ARRAY
        || t == CONFIG_TYPE_LIST;
#endif
}

// ── Setting hook wrappers ──

void *wrapper_config_setting_get_hook(const config_setting_t *setting) {
    return config_setting_get_hook(setting);
}

void *wrapper_config_get_hook(const config_t *config) {
#if LIBCONFIG_VER_MAJOR > 1 || (LIBCONFIG_VER_MAJOR == 1 && LIBCONFIG_VER_MINOR >= 8)
    return config_get_hook(config);
#else
    (void)config;
    return NULL;
#endif
}

// ── Include dir macro wrapper ──

const char *wrapper_config_get_include_dir(const config_t *config) {
#if (LIBCONFIG_VER_MAJOR > 1) || (LIBCONFIG_VER_MAJOR == 1 && LIBCONFIG_VER_MINOR >= 4)
    return config_get_include_dir(config);
#else
    return NULL;
#endif
}