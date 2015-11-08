/*
 * ChiRef.h
 *
 *  Created on: 2015年11月1日
 *      Author: Mickey
 */

#ifndef OBJREF_H_
#define OBJREF_H_

#include <memory>

class ObjRef {
	std::shared_ptr<void> p;
public:
	template<typename T>
	ObjRef(T * pi);
	virtual ~ObjRef();
};

#endif /* OBJREF_H_ */
