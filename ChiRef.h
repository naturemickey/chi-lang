/*
 * ChiRef.h
 *
 *  Created on: 2015Äê11ÔÂ1ÈÕ
 *      Author: Mickey
 */

#ifndef CHIREF_H_
#define CHIREF_H_

#include <memory>

class ChiRef {
	std::shared_ptr<int> p;
public:
	ChiRef(int * pi);
	virtual ~ChiRef();
};

#endif /* CHIREF_H_ */
