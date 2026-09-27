typedef unsigned long size_t;
#define NULL ((void *)0)
#define OPNG_PATH_EXTSEP_STR "."
size_t strlen(const char *);
char *strcpy(char *, const char *);
char *strcat(char *, const char *);

char *
opng_path_make_backup(char *buffer, size_t bufsize, const char *path)
{
    static const char bak_extname[] = OPNG_PATH_EXTSEP_STR "bak";

    if (strlen(path) + sizeof(bak_extname) > bufsize)
        return NULL;

#if defined OPNG_OS_DOS

    return opng_path_replace_ext(buffer, bufsize, path, bak_extname);

#else  /* OPNG_OS_UNIX and others */

    strcpy(buffer, path);
    strcat(buffer, bak_extname);
    return buffer;

#endif
}
