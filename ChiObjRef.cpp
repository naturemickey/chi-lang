/*
 * ChiRef.cpp
 *
 *  Created on: 2015年11月1日
 *      Author: Mickey
 */

#include "ChiObjRef.h"

template<typename T>
ChiObjRef::ChiObjRef(T * pi) : p(pi) {
}

ChiObjRef::~ChiObjRef() {
}
