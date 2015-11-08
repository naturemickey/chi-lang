/*
 * ParamRef.h
 *
 *  Created on: 2015年11月8日
 *      Author: Mickey
 */

#ifndef ARRAYREF_H_
#define ARRAYREF_H_

#include <vector>
#include <memory>

class ArrayRef {
	std::vector<std::shared_ptr<void>> p;
public:
	ArrayRef();
	virtual ~ArrayRef();
};

#endif /* ARRAYREF_H_ */
