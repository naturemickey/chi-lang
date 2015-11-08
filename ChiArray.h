/*
 * ParamRef.h
 *
 *  Created on: 2015年11月8日
 *      Author: Mickey
 */

#ifndef CHIARRAY_H_
#define CHIARRAY_H_

#include <vector>
#include <memory>
#include "ChiObj.h"

class ChiArray: public ChiObj {
	std::vector<std::shared_ptr<void>> p;
public:
	ChiArray();
	virtual ~ChiArray();
};

#endif /* CHIARRAY_H_ */
