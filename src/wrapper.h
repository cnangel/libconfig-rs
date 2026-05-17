#pragma once
#include <libconfig.h>

// Config lifecycle wrappers
config_t *wrapper_config_new(void);
void wrapper_config_free(config_t *config);

// Config error macro wrappers
const char *wrapper_config_error_text(const config_t *config);
const char *wrapper_config_error_file(const config_t *config);
int wrapper_config_error_line(const config_t *config);
int wrapper_config_error_type(const config_t *config);

// Config macro wrappers
config_setting_t *wrapper_config_root_setting(const config_t *config);
unsigned short wrapper_config_get_default_format(const config_t *config);
void wrapper_config_set_default_format(config_t *config, unsigned short format);
void wrapper_config_set_auto_convert(config_t *config, int flag);
unsigned short wrapper_config_get_tab_width(const config_t *config);
unsigned short wrapper_config_get_float_precision(const config_t *config);
int wrapper_config_get_auto_convert(const config_t *config);

// Setting macro wrappers
int wrapper_config_setting_type(const config_setting_t *setting);
const char *wrapper_config_setting_name(const config_setting_t *setting);
config_setting_t *wrapper_config_setting_parent(const config_setting_t *setting);
int wrapper_config_setting_is_root(const config_setting_t *setting);
unsigned int wrapper_config_setting_source_line(const config_setting_t *setting);
const char *wrapper_config_setting_source_file(const config_setting_t *setting);

// Setting type check wrappers
int wrapper_config_setting_is_group(const config_setting_t *setting);
int wrapper_config_setting_is_array(const config_setting_t *setting);
int wrapper_config_setting_is_list(const config_setting_t *setting);
int wrapper_config_setting_is_number(const config_setting_t *setting);
int wrapper_config_setting_is_scalar(const config_setting_t *setting);
int wrapper_config_setting_is_aggregate(const config_setting_t *setting);

// Hook wrappers
void *wrapper_config_setting_get_hook(const config_setting_t *setting);
void *wrapper_config_get_hook(const config_t *config);

// Include dir wrapper
const char *wrapper_config_get_include_dir(const config_t *config);