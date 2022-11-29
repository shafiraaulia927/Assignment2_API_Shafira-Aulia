<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET-Users</name>
   <tag></tag>
   <elementGuidId>3118ca53-9ce5-4f3d-bbd9-ca15e6da8f34</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
WS.verifyElementPropertyValue(response, '[0].id', '')
WS.verifyElementPropertyValue(response, '[0].name', '')
WS.verifyElementPropertyValue(response, '[0].username', '')
WS.verifyElementPropertyValue(response, '[0].email', '')
WS.verifyElementPropertyValue(response, '[0].address', '')
WS.verifyElementPropertyValue(response, '[0].address.street', '')
WS.verifyElementPropertyValue(response, '[0].address.suite', '')
WS.verifyElementPropertyValue(response, '[0].address.city', '')
WS.verifyElementPropertyValue(response, '[0].address.zipcode', '')
WS.verifyElementPropertyValue(response, '[0].address.geo', '')
WS.verifyElementPropertyValue(response, '[0].address.geo.lat', '')
WS.verifyElementPropertyValue(response, '[0].address.geo.lng', '')
WS.verifyElementPropertyValue(response, '[0].phone', '')
WS.verifyElementPropertyValue(response, '[0].website', '')
WS.verifyElementPropertyValue(response, '[0].company', '')
WS.verifyElementPropertyValue(response, '[0].company.name', '')
WS.verifyElementPropertyValue(response, '[0].company.catchPhrase', '')
WS.verifyElementPropertyValue(response, '[0].company.bs', '')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
