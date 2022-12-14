/* -*- Mode: IDL; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*- */
/*
 * The source for this IDL is found at https://www.khronos.org/registry/webgl/extensions/WEBGL_multi_draw/
 */
 [Exposed=(Window,Worker), LegacyNoInterfaceObject]
interface WEBGL_multi_draw {
  undefined multiDrawArraysWEBGL(
      GLenum mode,
      ([AllowShared] Int32Array or sequence<GLint>) firstsList, GLuint firstsOffset,
      ([AllowShared] Int32Array or sequence<GLsizei>) countsList, GLuint countsOffset,
      GLsizei drawcount);
  undefined multiDrawElementsWEBGL(
      GLenum mode,
      ([AllowShared] Int32Array or sequence<GLint>) countsList, GLuint countsOffset,
      GLenum type,
      ([AllowShared] Int32Array or sequence<GLsizei>) offsetsList, GLuint offsetsOffset,
      GLsizei drawcount);
  undefined multiDrawArraysInstancedWEBGL(
      GLenum mode,
      ([AllowShared] Int32Array or sequence<GLint>) firstsList, GLuint firstsOffset,
      ([AllowShared] Int32Array or sequence<GLsizei>) countsList, GLuint countsOffset,
      ([AllowShared] Int32Array or sequence<GLsizei>) instanceCountsList, GLuint instanceCountsOffset,
      GLsizei drawcount);
  undefined multiDrawElementsInstancedWEBGL(
      GLenum mode,
      ([AllowShared] Int32Array or sequence<GLint>) countsList, GLuint countsOffset,
      GLenum type,
      ([AllowShared] Int32Array or sequence<GLsizei>) offsetsList, GLuint offsetsOffset,
      ([AllowShared] Int32Array or sequence<GLsizei>) instanceCountsList, GLuint instanceCountsOffset,
      GLsizei drawcount);
};
