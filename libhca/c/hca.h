#ifndef __MY_HCA_H__

#define __MY_HCA_H__

typedef struct {
    char* guid;
    int lid;
} function_t;

typedef struct {
    char* description;
    char* serial_number;
    char* driver;
    function_t** functions;
} hca_t;

extern hca_t** list_hca();

#endif
