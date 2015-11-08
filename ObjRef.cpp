/*
 * ChiRef.cpp
 *
 *  Created on: 2015年11月1日
 *      Author: Mickey
 */

#include "ObjRef.h"

template<typename T>
ObjRef::ObjRef(T * pi) : p(pi) {
}

ObjRef::~ObjRef() {
}
