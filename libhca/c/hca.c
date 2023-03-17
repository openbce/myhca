#include <stdarg.h>
#include<stdlib.h>
#include <stdio.h>

#include <pci/pci.h>

#include "hca.h"

void die(char *msg, ...) {
    va_list args;

    va_start(args, msg);
    vfprintf(stderr, msg, args);

    exit(1);
}

hca_t** list_hca() {
    struct pci_filter filter;		/* Device filter */
    struct pci_access *pacc;
    struct device *first_dev;

    pacc = pci_alloc();

    pacc->error = die;
    pci_filter_init(pacc, &filter);


    return 0;
}
