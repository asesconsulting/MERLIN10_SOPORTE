<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>NameComparator_Zurich</name>
   <tag></tag>
   <elementGuidId>4728b3e9-e2ae-462f-9002-b67d4fd40a23</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.nameonline.comparator.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:nameComparatorNormalize2>
         &lt;!--Optional:-->
         &lt;nameComparatorNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapters> &lt;/customAdapters>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xNameComparator>
               &lt;!--Optional:-->
               &lt;leftName>Mariano&lt;/leftName>
               &lt;!--Optional:-->
               &lt;rigthName>Arturo Mariano&lt;/rigthName>
               &lt;!--Optional:-->
               &lt;leftLastName>Peral&lt;/leftLastName>
               &lt;!--Optional:-->
               &lt;rigthLastName>Peral&lt;/rigthLastName>
               &lt;!--Optional:-->
               &lt;leftGender> &lt;/leftGender>
               &lt;!--Optional:-->
               &lt;rigthGender> &lt;/rigthGender>
            &lt;/xNameComparator>
         &lt;/nameComparatorNormalize>
      &lt;/soap:nameComparatorNormalize2>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>nameComparatorNormalize2</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.NameComparator_ARG2</defaultValue>
      <description></description>
      <id>9951f527-dc5d-4278-be2b-4d45d3c1ab69</id>
      <masked>false</masked>
      <name>NameComparator_ARG2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyElementText(response, 'nameComparatorNormalize2Response.return.status', 'DS')
WS.verifyElementText(response, 'nameComparatorNormalize2Response.return.statusReason', '')
WS.verifyElementText(response, 'nameComparatorNormalize2Response.return.nNameComparator.differenceLevelName', '0.39333332')</verificationScript>
   <wsdlAddress>${NameComparator_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
