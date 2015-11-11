/*
 * ChiRef.h
 *
 *  Created on: 2015年11月1日
 *      Author: Mickey
 */

#ifndef CHIOBJREF_H_
#define CHIOBJREF_H_

#include <memory>
#include "ChiObj.h"

class ChiObjRef {
	std::shared_ptr<const ChiObj> p;
public:
	ChiObjRef(const ChiObj * pi);
	virtual ~ChiObjRef();
};

#endif /* CHIOBJREF_H_ */
