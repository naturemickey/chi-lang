/*
 * ChiRef.h
 *
 *  Created on: 2015年11月1日
 *      Author: Mickey
 */

#ifndef CHIOBJREF_H_
#define CHIOBJREF_H_

#include <memory>

class ChiObjRef {
	std::shared_ptr<void> p;
public:
	template<typename T>
	ChiObjRef(T * pi);
	virtual ~ChiObjRef();
};

#endif /* CHIOBJREF_H_ */
